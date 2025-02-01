# Fetch the latest release and download the appropriate binary
echo "selected arch: ${ARCH}"
RELEASE_META_DATA_URL="https://api.github.com/repos/sejoharp/worktimers/releases/latest"
BINARY_URL=$(curl -s ${RELEASE_META_DATA_URL} | jq -r ".assets[] | select(.name | contains(\"${ARCH}\")) | .browser_download_url")
echo "downloading ${BINARY_URL}"
curl -sLo worktimers ${BINARY_URL}
echo "make it executable"
chmod +x worktimers
echo "installing to ${HOME}/bin/worktimers"
mv worktimers ${HOME}/bin/