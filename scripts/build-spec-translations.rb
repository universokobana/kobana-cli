#!/usr/bin/env ruby
# frozen_string_literal: true

# Builds crates/kobana-cli/specs/i18n/<spec>.<lang>.json: every Portuguese
# `summary` and `description` of an embedded spec, mapped to its English or
# Spanish translation. The CLI embeds these maps and swaps the text in at
# startup, so `--help` and `kobana schema` follow the user's language.
#
# The translations come from each product's spec files, one per language (the
# same ones its user docs are generated from), read from the repo's `main`
# branch. Both specs are walked
# side by side: the text found at the same place in each is a pair.
#
#   ruby scripts/build-spec-translations.rb [path/to/sibling/repos]
#
# The default repo root is the directory that holds kobana-cli.

require "json"
require "yaml"
require "open3"

CLI = File.expand_path("..", __dir__)
ROOT = File.expand_path(ARGV[0] || File.join(CLI, ".."))
SPECS = File.join(CLI, "crates/kobana-cli/specs")
OUT = File.join(SPECS, "i18n")
LANGS = %w[en es].freeze

# embedded spec => [repo, path of the localized spec, with %s for the language]
SOURCES = {
  "banking-v1" => ["kobana-api-specs", "swagger/v1_0/kobana-api-v1_0-openapi-3_1.%s.json"],
  "banking-v2" => ["kobana-api-specs", "swagger/v2_0/kobana-api-v2_0-openapi-3_1.%s.json"],
  "billing-v1" => ["kobana-billing", "user-docs/openapi/openapi-v1-%s.yaml"],
  "dashboards-v1" => ["kobana-dashboards", "user-docs/openapi/openapi-v1-%s.yaml"],
  "dunning-v1" => ["kobana-dunning", "user-docs/openapi/openapi-v1-%s.yaml"],
  "finance-v1" => ["kobana-finance", "user-docs/static/openapi/openapi-v1-%s.yaml"],
  "inbox-v1" => ["kobana-inbox", "user-docs/openapi/openapi-v1-%s.yaml"]
}.freeze

TEXT_KEYS = %w[summary description].freeze

# kobana-api-specs is read from origin/main: a local checkout may lag behind
# the translate-specs workflow, which commits there on its own
def git_show(repo, path)
  ref = "main"
  if repo == "kobana-api-specs"
    ref = "origin/main"
    @fetched ||= system("git", "-C", File.join(ROOT, repo), "fetch", "-q", "origin", "main")
  end
  out, err, status = Open3.capture3("git", "-C", File.join(ROOT, repo), "show", "#{ref}:#{path}")
  abort "#{repo}:#{path}: #{err.strip}" unless status.success?
  out
end

def load_spec(text, path)
  return JSON.parse(text) if path.end_with?(".json")

  # The system Ruby (2.6) predates unsafe_load, where load is already unsafe
  YAML.respond_to?(:unsafe_load) ? YAML.unsafe_load(text) : YAML.load(text)
end

# A path may be spelled differently on the other side: the embedded finance
# spec carries /v1 in its paths while the published one keeps it in `servers`,
# and the docs may name a path parameter differently (`{id}` for `{uid}`).
def counterpart(other, key)
  return other[key] if other.key?(key)
  return unless key.start_with?("/")

  shape = ->(path) { path.sub(%r{\A/v\d+(?=/)}, "").gsub(/\{[^}]+\}/, "{}") }
  match = other.keys.find { |k| k.start_with?("/") && shape.call(k) == shape.call(key) }
  other[match] if match
end

def walk(pt, other, pairs)
  case pt
  when Hash
    return unless other.is_a?(Hash)

    pt.each do |key, value|
      match = counterpart(other, key)
      if TEXT_KEYS.include?(key) && value.is_a?(String)
        pairs[value] << match if match.is_a?(String) && match != value && !match.strip.empty?
      else
        walk(value, match, pairs)
      end
    end
  when Array
    return unless other.is_a?(Array) && other.size == pt.size

    pt.each_with_index { |item, i| walk(item, other[i], pairs) }
  end
end

def count_texts(node)
  case node
  when Hash
    node.sum { |k, v| TEXT_KEYS.include?(k) && v.is_a?(String) ? 1 : count_texts(v) }
  when Array then node.sum { |v| count_texts(v) }
  else 0
  end
end

Dir.mkdir(OUT) unless Dir.exist?(OUT)

SOURCES.each do |name, (repo, pattern)|
  pt = JSON.parse(File.read(File.join(SPECS, "#{name}.json")))
  total = count_texts(pt)

  LANGS.each do |lang|
    path = format(pattern, lang)
    pairs = Hash.new { |h, k| h[k] = [] }
    walk(pt, load_spec(git_show(repo, path), path), pairs)

    # The same Portuguese text can be translated differently in two places;
    # keep the most common translation
    map = pairs.transform_values { |list| list.group_by(&:itself).max_by { |_, same| same.size }.first }
    File.write(File.join(OUT, "#{name}.#{lang}.json"), JSON.pretty_generate(map.sort.to_h) + "\n")
    puts format("%-14s %s  %5d translations (%d texts in the spec)", name, lang, map.size, total)
  end
end
