#!/usr/bin/env bash
#
# publish-linux-repo.sh — sign the .rpm files and assemble GPG-signed apt + dnf
# (and zypper) repositories plus a static index, ready for GitHub Pages.
#
# Modeled on kia-desktop's script of the same name. The private key never
# touches stdout: it is imported into a throwaway GNUPGHOME from
# $GPG_PRIVATE_KEY (armored) and the passphrase is read from a file.
#
# Required env:
#   GPG_PRIVATE_KEY   armored private key (ASCII)
#   GPG_PASSPHRASE    passphrase for that key
#   PAGES_BASE_URL    public base, e.g. https://universokobana.github.io/kobana-cli
# Arg 1: directory searched (recursively) for the .deb/.rpm built by CI.
#
# The .rpm files are signed IN PLACE, so whatever uploads that directory to the
# release (and attests it) ships the signed ones. Produces ./site.
#
# Tools needed: gpg, rpm, createrepo-c, dpkg-dev, apt-utils, gzip.
set -euo pipefail

PKG_DIR="${1:?usage: publish-linux-repo.sh <dir with .deb/.rpm>}"
: "${GPG_PRIVATE_KEY:?}"; : "${GPG_PASSPHRASE:?}"; : "${PAGES_BASE_URL:?}"

WORK="$(mktemp -d)"
SITE="$PWD/site"
mkdir -p "$SITE"

# --- isolated keyring; short path so the gpg-agent socket fits ---
export GNUPGHOME="$(mktemp -d /tmp/kgpg.XXXXXX)"
chmod 700 "$GNUPGHOME"
PASS_FILE="$GNUPGHOME/pass"
( umask 077; printf '%s' "$GPG_PASSPHRASE" > "$PASS_FILE" )
# The secret is normally the armored key; a base64 copy of it is accepted too.
KEY="$GPG_PRIVATE_KEY"
case "$KEY" in
  *"BEGIN PGP"*) ;;
  *) KEY="$(printf '%s' "$KEY" | base64 -d 2>/dev/null || true)" ;;
esac
# gpg prints key ids and user ids here, never the secret material.
if ! printf '%s\n' "$KEY" | gpg --batch --import; then
  echo "::error::GPG_PRIVATE_KEY is not an armored PGP private key (or a base64 copy of one); export it with: gpg --armor --export-secret-keys <id>" >&2
  exit 1
fi
FPR="$(gpg --list-secret-keys --with-colons | awk -F: '/^fpr:/{print $10; exit}')"
[ -n "$FPR" ] || { echo "::error::no secret key found in GPG_PRIVATE_KEY (is it the public key?)" >&2; exit 1; }
echo "Imported signing key $FPR"
echo ok | gpg --batch --yes --pinentry-mode loopback --passphrase-file "$PASS_FILE" \
    --clearsign -u "$FPR" >/dev/null \
  || { echo "::error::cannot sign with the key: is GPG_PASSPHRASE correct?" >&2; exit 1; }
# An expiring key breaks every install on the expiry date (cli/cli#9569, #13118).
gpg --list-keys --with-colons "$FPR" | awk -F: '/^pub:/{ if ($7 != "") print "::warning::signing key expires (epoch " $7 ")"; else print "signing key has no expiry" }'

gpg_sign() { # detached ascii sig: gpg_sign <file> <out.asc>
  gpg --batch --yes --pinentry-mode loopback --passphrase-file "$PASS_FILE" \
      --armor --detach-sign -u "$FPR" -o "$2" "$1"
}

mapfile -t RPMS < <(find "$PKG_DIR" -name '*.rpm' | sort)
mapfile -t DEBS < <(find "$PKG_DIR" -name '*.deb' | sort)
echo "found ${#DEBS[@]} deb, ${#RPMS[@]} rpm"
[ ${#DEBS[@]} -gt 0 ] && [ ${#RPMS[@]} -gt 0 ] || { echo "no packages in $PKG_DIR" >&2; exit 1; }

# ============================================================================
# 1. RPM — embed a GPG signature (dnf/zypper verify per-package), in place
# ============================================================================
# Keep rpm's own %__gpg_sign_cmd (its layout changed between 4.x and 6.x) and
# give gpg what it needs through the throwaway keyring's gpg.conf instead.
cat >> "$GNUPGHOME/gpg.conf" <<EOM
batch
yes
pinentry-mode loopback
passphrase-file $PASS_FILE
no-secmem-warning
EOM
cat > "$HOME/.rpmmacros" <<EOM
%_signature gpg
%__gpg /usr/bin/gpg
%_gpg_name $FPR
%_gpg_digest_algo sha256
EOM
for r in "${RPMS[@]}"; do
  rpm --addsign "$r"
  # The header signature carries the key id on rpm 4.x and 6.x (`rpm -qi` does not).
  [ -n "$(rpm -qp --qf '%{RSAHEADER:pgpsig}' "$r" 2>/dev/null)" ] || { echo "rpm not signed: $r" >&2; exit 1; }
  echo "signed $(basename "$r")"
done

# ============================================================================
# 2. Public key (armored, and dearmored for apt's signed-by keyring)
# ============================================================================
gpg --armor --export "$FPR" > "$SITE/kobana.asc"
gpg --export "$FPR" > "$SITE/kobana.gpg"

# ============================================================================
# 3. APT repository (dists/stable, component main, arches amd64+arm64)
# ============================================================================
APT="$SITE/apt"
mkdir -p "$APT/pool/main"
cp "${DEBS[@]}" "$APT/pool/main/"
( cd "$APT"
  for arch in amd64 arm64; do
    d="dists/stable/main/binary-$arch"; mkdir -p "$d"
    dpkg-scanpackages --arch "$arch" pool /dev/null > "$d/Packages"
    gzip -9kf "$d/Packages"
  done
  cat > "$WORK/apt-release.conf" <<EOM
APT::FTPArchive::Release::Origin "Kobana";
APT::FTPArchive::Release::Label "Kobana CLI";
APT::FTPArchive::Release::Suite "stable";
APT::FTPArchive::Release::Codename "stable";
APT::FTPArchive::Release::Architectures "amd64 arm64";
APT::FTPArchive::Release::Components "main";
APT::FTPArchive::Release::Description "Kobana CLI";
EOM
  apt-ftparchive -c "$WORK/apt-release.conf" release dists/stable > dists/stable/Release
  gpg --batch --yes --pinentry-mode loopback --passphrase-file "$PASS_FILE" \
      --clearsign -u "$FPR" -o dists/stable/InRelease dists/stable/Release
  gpg --batch --yes --pinentry-mode loopback --passphrase-file "$PASS_FILE" \
      --armor --detach-sign -u "$FPR" -o dists/stable/Release.gpg dists/stable/Release
)
echo "apt repo built"

# ============================================================================
# 4. DNF/YUM/zypper repository (createrepo_c + signed repomd.xml)
# ============================================================================
RPMREPO="$SITE/rpm"
mkdir -p "$RPMREPO"
cp "${RPMS[@]}" "$RPMREPO/"
createrepo_c "$RPMREPO"
gpg_sign "$RPMREPO/repodata/repomd.xml" "$RPMREPO/repodata/repomd.xml.asc"
cat > "$RPMREPO/kobana.repo" <<EOM
[kobana]
name=Kobana CLI
baseurl=$PAGES_BASE_URL/rpm
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey=$PAGES_BASE_URL/kobana.asc
EOM
echo "rpm repo built"

# ============================================================================
# 5. Static index with copy-paste install instructions
# ============================================================================
cat > "$SITE/index.html" <<EOM
<!doctype html><html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Kobana CLI — Linux packages</title>
<style>body{font-family:system-ui,sans-serif;max-width:760px;margin:2rem auto;padding:0 1rem;background:#000;color:#FDFDFB}
code,pre{background:#111;color:#D3FD54}pre{padding:1rem;border-radius:8px;overflow:auto}a{color:#A630DA}</style>
</head><body>
<h1>Kobana CLI — Linux packages</h1>
<p>Signed apt, dnf and zypper repositories. Key fingerprint: <code>$FPR</code></p>
<h2>Debian / Ubuntu (apt)</h2>
<pre>sudo install -d -m 0755 /etc/apt/keyrings
curl -fsSL $PAGES_BASE_URL/kobana.gpg | sudo tee /etc/apt/keyrings/kobana.gpg >/dev/null
echo "deb [signed-by=/etc/apt/keyrings/kobana.gpg] $PAGES_BASE_URL/apt stable main" \\
  | sudo tee /etc/apt/sources.list.d/kobana.list
sudo apt update && sudo apt install kobana</pre>
<h2>Fedora / RHEL / CentOS (dnf)</h2>
<pre>sudo rpm --import $PAGES_BASE_URL/kobana.asc
sudo curl -fsSL -o /etc/yum.repos.d/kobana.repo $PAGES_BASE_URL/rpm/kobana.repo
sudo dnf install kobana</pre>
<h2>openSUSE / SUSE (zypper)</h2>
<pre>sudo rpm --import $PAGES_BASE_URL/kobana.asc
sudo zypper addrepo $PAGES_BASE_URL/rpm/kobana.repo
sudo zypper install kobana</pre>
<p>Check the fingerprint above against <code>gpg --show-keys kobana.asc</code> before trusting the key.</p>
</body></html>
EOM

# never leave key material behind
rm -rf "$GNUPGHOME" "$WORK"
echo "site/ ready:"; find "$SITE" -type f | sort
