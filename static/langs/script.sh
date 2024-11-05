#!/bin/bash

declare -A url_map=(
    ["https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"]="rust.svg"
    ["https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white"]="go.svg"
    ["https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white"]="c.svg"
    ["https://img.shields.io/badge/c%23-%23239120.svg?style=for-the-badge&logo=csharp&logoColor=white"]="csharp.svg"
    ["https://img.shields.io/badge/swift-%23F15035.svg?style=for-the-badge&logo=swift&logoColor=white"]="swift.svg"
    ["https://img.shields.io/badge/java-%23ED8B00.svg?style=for-the-badge&logo=openjdk&logoColor=white"]="java.svg"
    ["https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E"]="javascript.svg"
    ["https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white"]="lua.svg"
    ["https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54"]="python.svg"
)

for url in "${!url_map[@]}"; do
    filename="${url_map[$url]}"
    echo "downloading $filename..."
    curl -s -o "$filename" "$url"
    if [ $? -eq 0 ]; then
        echo "succesfully downloaded $filename"
    else
        echo "failed to download $filename"
    fi
done

echo "done"
