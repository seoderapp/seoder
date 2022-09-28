# Engines

Engines for running different aspects of the program and relevant data related to it.

## Campaigns

Campaigns can be used to determine stats on a group by a name.

Each new campaign gets a new that can be created using the `jsoncrawler_client`
and the `seoder_web` server.

```sh
# start the socket server
RUST_LOG=info cargo run --package seoder_web
# start the client
RUST_LOG=info cargo run --package jsoncrawler_client
```

### API

Run the following commands in the web socket client.

Replace `$mycampaign` with the campaign name.

#### create-campaign

Create a new campaign folder to measure stats performed.

`create-campaign $mycampaign`

#### run-campaign

Run a campaign based on the central configs.

`run-campaign $mycampaign`

#### stop-campaign

Stop a mid flight campaign crawl [todo]

`run-campaign $mycampaign`

#### delete-campaign

Delete a campaign from tracking [todo]

`run-campaign $mycampaign`
