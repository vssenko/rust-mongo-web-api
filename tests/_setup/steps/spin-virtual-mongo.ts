import test from '../../ava';
import { MongoMemoryServer } from 'mongodb-memory-server';

async function _createMongo() {
  return MongoMemoryServer.create();
}

test.serial.before('Starting virtual MongoDB', async (t) => {
  const mongoServer = await _createMongo();
  t.context.mongodb = mongoServer;
});
