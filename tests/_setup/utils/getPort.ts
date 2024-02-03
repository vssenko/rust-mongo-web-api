import net from 'net';

export function getFreePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.on('error', (e) => reject(e));
    server.listen(0, () => {
      const { port } = <net.AddressInfo>server.address();
      server.on('close', () => resolve(port));
      server.close();
    });
  });
}
