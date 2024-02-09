import { MongoMemoryServer } from 'mongodb-memory-server';

let mongoServer = null;
let mongourl = null;

export async function createMongo() {
  mongoServer = await MongoMemoryServer.create();
  mongourl = mongoServer.getUri();

  console.log(`mongo: created with url "${mongourl}"`)
}

export async function stopMongo() {
  await mongoServer.stop();
  mongoServer = null;
  mongourl = null;

  console.log('mongo: stopped');
}

export function getUrl() {
  if (!mongourl) throw new Error('Mongo server is not started');
  return mongourl;
}

export default {
  createMongo,
  stopMongo,
  getUrl
}