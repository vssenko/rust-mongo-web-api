import net from 'net';
import api from './api.js';
import mongo from './mongo.js';


function _getFreePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.on('error', (e) => reject(e));
    server.listen(0, () => {
      const { port } = server.address();
      server.on('close', () => resolve(port));
      server.close();
    });
  });
}


export async function bootstrap() {
  await mongo.createMongo();
  await api.startApi({
    mongourl: mongo.getUrl(),
    port: await _getFreePort()
  })
}

export async function shutdown() {
  await api.stopApi();
  await mongo.stopMongo();
}


export default {
  bootstrap,
  shutdown
};
