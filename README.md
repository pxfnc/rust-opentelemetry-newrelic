# rustのログをopentelemetryでnewrelicとjaegerで見る 


```sh
# config/otelcol.yamlのnew_relic_api_keyを実際のapiで編集しておく

docker compose build
docker compose up 

# サイコロAPIを呼ぶと、otelcol:4317に向かってotlpでtraceを送る
curl http://localhost:8000/roll_dice

# jaegerでtraceを覗いてみる
open http://localhost:55679
# ./config/otelcol.yamlの設定に従って、jaegerとnewrelicにデータを送る
```

