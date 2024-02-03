import test from '../../ava';
import api from '../api';

test.after.always('MongoDB & Server Cleanup', async (t) => {
  try {
    await api.stopApi();
    await t.context.mongodb?.stop();
  } catch (e) {
    console.error('Could not exit test gracefully: ', e);
    process.exit(-1);
  }
});
