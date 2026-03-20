#!/usr/bin/env bash
# scripts/fetch-fonts.sh
# Fetches Inter + JetBrains Mono woff2 files by parsing live Google Fonts CSS.
# Run from repo root: bash scripts/fetch-fonts.sh
# Commit the resulting static/fonts/*.woff2 files.
set -euo pipefail

FONT_DIR="static/fonts"
mkdir -p "$FONT_DIR"

UA="Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"

fetch_font_css() {
  curl -fsSL -A "$UA" "$1"
}

download_woff2() {
  local name="$1" url="$2" dest="${FONT_DIR}/${name}.woff2"
  if [[ -f "$dest" ]]; then echo "  [skip] ${name}.woff2 already exists"; return; fi
  curl -fsSL "$url" -o "$dest"
  echo "  [ok]   ${name}.woff2"
}

echo "==> Fetching Inter CSS..."
INTER_CSS=$(fetch_font_css \
  "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap")

echo "==> Fetching JetBrains Mono CSS..."
JBM_CSS=$(fetch_font_css \
  "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&display=swap")

extract_urls() {
  grep -oP "(?<=url\()[^)]+(?=\))" <<< "$1" | grep "\.woff2"
}

INTER_URLS=($(extract_urls "$INTER_CSS"))
JBM_URLS=($(extract_urls "$JBM_CSS"))

echo "==> Found ${#INTER_URLS[@]} Inter URLs, ${#JBM_URLS[@]} JetBrains Mono URLs"

INTER_NAMES=(inter-400 inter-500 inter-600 inter-700 inter-800)
JBM_NAMES=(jbmono-400 jbmono-500 jbmono-600)

[[ ${#INTER_URLS[@]} -ge 5 ]] || { echo "ERROR: Expected 5 Inter URLs"; exit 1; }
[[ ${#JBM_URLS[@]}   -ge 3 ]] || { echo "ERROR: Expected 3 JBM URLs";   exit 1; }

echo "==> Downloading Inter..."
for i in "${!INTER_NAMES[@]}"; do download_woff2 "${INTER_NAMES[$i]}" "${INTER_URLS[$i]}"; done

echo "==> Downloading JetBrains Mono..."
for i in "${!JBM_NAMES[@]}"; do download_woff2 "${JBM_NAMES[$i]}" "${JBM_URLS[$i]}"; done

echo "==> Verifying..."
FAIL=0
for name in "${INTER_NAMES[@]}" "${JBM_NAMES[@]}"; do
  size=$(wc -c < "${FONT_DIR}/${name}.woff2" 2>/dev/null || echo 0)
  if [[ $size -lt 10000 ]]; then
    echo "  [FAIL] ${name}.woff2 — ${size} bytes (likely 404)"; FAIL=1
  else
    echo "  [ok]   ${name}.woff2 — ${size} bytes"
  fi
done

[[ $FAIL -eq 0 ]] || { echo "Font download failed. See errors above."; exit 1; }

echo ""
echo "Done. Next:"
echo "  git add static/fonts/*.woff2"
echo "  git commit -m 'feat: self-host Inter and JetBrains Mono'"
