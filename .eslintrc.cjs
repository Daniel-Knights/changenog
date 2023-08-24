// @ts-check
/* eslint-disable @typescript-eslint/no-var-requires */
const { defineConfig } = require("eslint-define-config");

module.exports = defineConfig({
  root: true,
  env: {
    node: true,
  },
  extends: [
    "airbnb-base",
    "eslint:recommended",
    "prettier",
    "plugin:n/recommended",
    "plugin:@typescript-eslint/recommended",
  ],
  parserOptions: {
    parser: "@typescript-eslint/parser",
    ecmaVersion: "latest",
    sourceType: "module",
  },
  plugins: ["@typescript-eslint", "n", "prettier"],
  ignorePatterns: ["dist", "node_modules"],
  rules: {
    "no-console": "off",
    "no-param-reassign": [2, { props: false }],
    "no-new": "off",
    "no-restricted-syntax": "off",
    "no-cond-assign": ["error", "except-parens"],
    "prettier/prettier": ["error", { endOfLine: "auto" }],
    "arrow-body-style": "off",
    "consistent-return": "off",
    "lines-between-class-members": ["error", "always", { exceptAfterSingleLine: true }],

    "import/prefer-default-export": "off",
    "import/extensions": "off",
    "import/no-extraneous-dependencies": [
      "error",
      { devDependencies: [".eslintrc.cjs"] },
    ],

    "@typescript-eslint/no-non-null-assertion": "off",
    "@typescript-eslint/consistent-type-imports": [
      "error",
      { disallowTypeAnnotations: false },
    ],
    "@typescript-eslint/explicit-function-return-type": "error",

    "n/shebang": "off",
  },
  settings: {
    "import/resolver": {
      typescript: true,
    },
  },
});
