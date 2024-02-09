import { MongoClient, Db } from "mongodb";
import { MongoMemoryServer } from "mongodb-memory-server";

let mongoServer = null;
let mongourl = null;
let mongoClientPromise = null;

export async function createMongo() {
  mongoServer = await MongoMemoryServer.create();
  mongourl = mongoServer.getUri();

  console.log(`mongo: created with url "${mongourl}"`);
}

export async function stopMongo() {
  if (mongoClientPromise) {
    const mongoClient = await mongoClientPromise;
    await mongoClient.close();
  }
  await mongoServer.stop();
  mongoServer = null;
  mongourl = null;

  console.log("mongo: stopped");
}

export function getUrl() {
  if (!mongourl) throw new Error("Mongo server is not started");
  return mongourl;
}

/**
 *
 * @returns {Db}
 */
export async function getDatabase() {
  if (!mongoClientPromise) {
    mongoClientPromise = new Promise(async (resolve, reject) => {
      try {
        const mc = new MongoClient(mongourl);
        await mc.connect();
        resolve(mc);
      } catch (e) {
        reject(e);
      }
    });
  }

  const mongoClient = await mongoClientPromise;

  return mongoClient.db("rust-mongo-web-api");
}

export default {
  createMongo,
  stopMongo,
  getUrl,
  getDatabase,
};
