module.exports = {
  extensions: ['ts'],
  require: [
    'ts-node/register',
    './tests/_setup/integration.setup.ts'
  ],
  files: [
    'tests/**/*.test.ts',
  ],
  verbose: true,
  failFast: false,
  environmentVariables: {
    NODE_ENV: 'test',
    ENV: 'test',
  },
  timeout: '60s',
  concurrency: 3,
  nodeArguments: [],
};
