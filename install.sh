#!/usr/bin/env bash

echo "selected arch: ${ARCH}"

ARCH=$(uname -m)
if [ "${ARCH}" == "x86_64" ]; then
  ARCH="x86_64"
elif [ "${ARCH}" == "arm64" ]; then
  ARCH="aarch64"
else
  echo "Unsupported architecture: ${ARCH}"
  exit 1
fi

RELEASE_META_DATA_URL="https://api.github.com/repos/sejoharp/worktimers/releases/latest"
BINARY_URL=$(curl -s ${RELEASE_META_DATA_URL} | jq -r ".assets[] | select(.name | contains(\"${ARCH}\")) | .browser_download_url")
echo "downloading ${BINARY_URL}"
curl -sLo worktimers ${BINARY_URL}
echo "make it executable"
chmod +x worktimers
echo "installing to ${HOME}/bin/worktimers"
mv worktimers ${HOME}/bin/