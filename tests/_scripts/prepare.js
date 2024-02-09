import { MongoMemoryServer } from "mongodb-memory-server";

async function main() {
  try {
    console.log("Prepare tests: Start warming mongo memory server...");
    const mongod = await MongoMemoryServer.create();
    console.log("Prepare tests: Mongo memory server created. Getting url...");
    const uri = await mongod.getUri();
    console.log(`Prepare tests: Mongo memory server url: ${uri}.`);
    console.log("Prepare tests: Finished preparing process.");
    process.exit(0);
  } catch (e) {
    console.log("Prepare tests: Failed to spin server: ", e);
    process.exit(-1);
  }
}

main();
