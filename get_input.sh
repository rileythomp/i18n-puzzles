cur_dir=${PWD##*/}
curl "https://i18n-puzzles.com/puzzle/${cur_dir:3}/input" \
  -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7' \
  -H 'Accept-Language: en' \
  -H 'Cache-Control: max-age=0' \
  -H 'Connection: keep-alive' \
  -b 'csrftoken=oxYyCGVNWq4hSo3rkzOxJjCDXpq3rjF7; sessionid=meur0jsp4qpvi9fxbzbxfpuc9gtws3us' \
  -H 'DNT: 1' \
  -H 'Referer: https://i18n-puzzles.com/puzzle/1/' \
  -H 'Sec-Fetch-Dest: document' \
  -H 'Sec-Fetch-Mode: navigate' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'Sec-Fetch-User: ?1' \
  -H 'Upgrade-Insecure-Requests: 1' \
  -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36' \
  -H 'sec-ch-ua: "Chromium";v="134", "Not:A-Brand";v="24", "Google Chrome";v="134"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "macOS"' > in.txt
