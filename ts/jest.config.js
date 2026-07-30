module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  // ts-jest compiles with tsconfig.json unless told which one to use, and that config excludes
  // tests/. tsconfig.test.json exists for exactly this and was wired to nothing, so the test files
  // were compiled under a config that does not describe them.
  transform: {
    '^.+\\.tsx?$': ['ts-jest', { tsconfig: 'tsconfig.test.json' }],
  },
  roots: ['<rootDir>/tests'],
  testMatch: ['**/*.test.ts'],
  collectCoverageFrom: [
    'src/**/*.ts',
    '!src/**/*.d.ts',
  ],
  coverageDirectory: 'coverage',
  coverageReporters: ['text', 'lcov', 'html'],
};