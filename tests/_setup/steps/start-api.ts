import test from '../../ava';
import api from '../api';

test.serial.before('Starting API', async (t) => {
  await api.startApi({
    mongourl: t.context.mongodb.getUri()
  });

  t.context.api = api.getApi;
});
