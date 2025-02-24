// @ts-check
/* eslint-disable n/no-unpublished-import */
import js from "@eslint/js";
import eslintConfigPrettier from "eslint-config-prettier";
// @ts-expect-error - eslint-plugin-import does not have types
import eslintPluginImport from "eslint-plugin-import";
import eslintPluginN from "eslint-plugin-n";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";
import typescriptEslint, { configs as typescriptEslintConfigs } from "typescript-eslint";

export default typescriptEslint.config(
  js.configs.recommended,
  eslintConfigPrettier,
  eslintPluginImport.flatConfigs.recommended,
  eslintPluginImport.flatConfigs.typescript,
  eslintPluginN.configs["flat/recommended"],
  ...typescriptEslintConfigs.recommended,
  {
    ignores: ["**/node_modules/**/*", "**/dist/**/*"],
  },
  {
    linterOptions: {
      reportUnusedDisableDirectives: "warn",
    },
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
        parser: "@typescript-eslint/parser",
      },
      globals: {
        ...globals.node,
      },
    },
    settings: {
      "import/resolver": {
        typescript: true,
        node: true,
      },
    },
    rules: {
      "linebreak-style": ["error", process.platform === "win32" ? "windows" : "unix"],

      "prettier/prettier": 1,

      "no-console": process.env.NODE_ENV === "production" ? "warn" : "off",
      "no-debugger": process.env.NODE_ENV === "production" ? "error" : "off",

      "n/no-process-exit": "error",
      "n/hashbang": [
        "error",
        {
          convertPath: { "src/**/*.ts": ["^src/(.+).ts$", "dist/$1.js"] },
        },
      ],

      "@typescript-eslint/no-shadow": "error",
      "@typescript-eslint/no-unused-vars": "error",
      "@typescript-eslint/no-explicit-any": "warn",
      "@typescript-eslint/explicit-module-boundary-types": "off",
      "@typescript-eslint/no-non-null-assertion": "off",
      "@typescript-eslint/no-use-before-define": [
        "error",
        { functions: false, classes: false, variables: false },
      ],

      "import/no-unresolved": ["error", { caseSensitive: true }],
      "import/named": "off",
      "import/default": "error",
      "import/export": "error",
      "import/no-named-as-default": "error",
      "import/no-named-as-default-member": "error",
      "import/no-mutable-exports": "error",
      "import/no-commonjs": "error",
      "import/no-amd": "error",
      "import/no-nodejs-modules": "off",
      "import/first": "error",
      "import/no-duplicates": "error",
      "import/no-namespace": "off",
      "import/newline-after-import": "error",
      "import/no-absolute-path": "error",
      "import/no-internal-modules": "off",
      "import/no-webpack-loader-syntax": "error",
      "import/no-unassigned-import": ["error", { allow: ["**/*.{s,}css"] }],
      "import/no-named-default": "error",
      "import/exports-last": "off",
      "import/group-exports": "off",
      "import/no-default-export": "off",
      "import/no-named-export": "off",
      "import/no-self-import": "error",
      "import/no-cycle": "off",
      "import/no-useless-path-segments": "error",
      "import/no-relative-parent-imports": "off",
      "import/no-unused-modules": "error",
      "import/no-import-module-exports": "error",
      "import/extensions": "off",
      "import/prefer-default-export": "off",
      "import/no-relative-packages": "error",
      "import/no-extraneous-dependencies": [
        "error",
        {
          devDependencies: ["eslint.config.mjs", "scripts/**"],
        },
      ],

      "array-callback-return": ["error", { allowImplicit: true }],
      "arrow-body-style": "off",
      "arrow-parens": "error",
      camelcase: ["error", { properties: "never", ignoreDestructuring: false }],
      "capitalized-comments": "off",
      "class-methods-use-this": "error",
      "comma-dangle": "off",
      "consistent-return": "off",
      "constructor-super": "error",
      curly: ["error", "multi-line"],
      "default-case": "off",
      "default-param-last": "error",
      "dot-notation": "error",
      eqeqeq: "error",
      "for-direction": "error",
      "func-names": "off",
      "func-style": ["error", "declaration", { allowArrowFunctions: true }],
      "getter-return": ["error", { allowImplicit: true }],
      "grouped-accessor-pairs": "error",
      "guard-for-in": "error",
      "lines-around-comment": "off",
      "lines-between-class-members": "off",
      "max-classes-per-file": "off",
      "max-nested-callbacks": ["error", 4],
      "max-params": ["error", 4],
      "max-statements": "off",
      "multiline-comment-style": "off",
      "multiline-ternary": "off",
      "new-cap": "off",
      "newline-before-return": "error",
      "no-alert": "warn",
      "no-array-constructor": "error",
      "no-async-promise-executor": "error",
      "no-await-in-loop": "off",
      "no-bitwise": "error",
      "no-case-declarations": "error",
      "no-catch-shadow": "error",
      "no-class-assign": "error",
      "no-cond-assign": "error",
      "no-const-assign": "error",
      "no-constant-condition": "error",
      "no-constructor-return": "error",
      "no-continue": "off",
      "no-control-regex": "error",
      "no-dupe-args": "error",
      "no-dupe-class-members": "error",
      "no-dupe-else-if": "error",
      "no-dupe-keys": "error",
      "no-duplicate-case": "error",
      "no-duplicate-imports": "error",
      "no-else-return": ["error", { allowElseIf: false }],
      "no-empty": "error",
      "no-empty-character-class": "error",
      "no-empty-function": "error",
      "no-empty-pattern": "error",
      "no-ex-assign": "error",
      "no-extend-native": "error",
      "no-extra-bind": "error",
      "no-extra-boolean-cast": "error",
      "no-extra-label": "error",
      "no-extra-semi": "error",
      "no-fallthrough": "error",
      "no-floating-decimal": "error",
      "no-func-assign": "error",
      "no-global-assign": "error",
      "no-implicit-coercion": ["off", { allow: ["!!"] }],
      "no-import-assign": "error",
      "no-inline-comments": "off",
      "no-inner-declarations": "error",
      "no-invalid-regexp": "error",
      "no-irregular-whitespace": "error",
      "no-label-var": "error",
      "no-lone-blocks": "error",
      "no-lonely-if": "error",
      "no-loop-func": "error",
      "no-loss-of-precision": "error",
      "no-misleading-character-class": "error",
      "no-mixed-spaces-and-tabs": "error",
      "no-multi-assign": "error",
      "no-negated-condition": "off",
      "no-nested-ternary": "error",
      "no-new": "error",
      "no-new-object": "error",
      "no-new-symbol": "error",
      "no-param-reassign": "error",
      "no-plusplus": "error",
      "no-promise-executor-return": "off",
      "no-prototype-builtins": "error",
      "no-redeclare": "error",
      "no-regex-spaces": "error",
      "no-restricted-syntax": [
        "error",
        {
          selector: "ForInStatement",
          message:
            "for..in loops iterate over the entire prototype chain, which is virtually never what you want. Use Object.{keys,values,entries}, and iterate over the resulting array.",
        },
        {
          selector: "LabeledStatement",
          message:
            "Labels are a form of GOTO; using them makes code confusing and hard to maintain and understand.",
        },
        {
          selector: "WithStatement",
          message:
            "`with` is disallowed in strict mode because it makes code impossible to predict and optimize.",
        },
      ],
      "no-return-assign": "error",
      "no-return-await": "error",
      "no-self-compare": "error",
      "no-sequences": "error",
      "no-setter-return": "error",
      "no-shadow": "error",
      "no-shadow-restricted-names": "error",
      "no-sparse-arrays": "error",
      "no-tabs": "error",
      "no-template-curly-in-string": "warn",
      "no-ternary": "off",
      "no-this-before-super": "error",
      "no-throw-literal": "error",
      "no-undef": "error",
      "no-undefined": "off",
      "no-underscore-dangle": ["error", { allow: ["__dirname", "__filename"] }],
      "no-unexpected-multiline": "error",
      "no-unneeded-ternary": ["error", { defaultAssignment: false }],
      "no-unreachable": "error",
      "no-unreachable-loop": "error",
      "no-unsafe-finally": "error",
      "no-unsafe-negation": "error",
      "no-unsafe-optional-chaining": ["error", { disallowArithmeticOperators: true }],
      "no-unused-expressions": "error",
      "no-unused-labels": "error",
      "no-unused-vars": "off", // Handled by @typescript-eslint/no-unused-vars
      "no-use-before-define": "off", // Handled by @typescript-eslint/no-use-before-define
      "no-useless-backreference": "error",
      "no-useless-catch": "error",
      "no-useless-computed-key": "error",
      "no-useless-concat": "error",
      "no-useless-constructor": "error",
      "no-useless-escape": "off",
      "no-useless-rename": "error",
      "no-useless-return": "error",
      "no-var": "error",
      "object-curly-newline": "off",
      "object-shorthand": "error",
      "operator-assignment": "error",
      "operator-linebreak": "off",
      "prefer-arrow-callback": "error",
      "prefer-const": "error",
      "prefer-destructuring": "error",
      "prefer-exponentiation-operator": "error",
      "prefer-numeric-literals": "error",
      "prefer-object-spread": "error",
      "prefer-promise-reject-errors": ["error", { allowEmptyReject: true }],
      "prefer-reflect": "off",
      "prefer-rest-params": "error",
      "prefer-spread": "error",
      "prefer-template": "error",
      "require-await": "warn",
      "require-yield": "error",
      "rest-spread-spacing": ["error", "never"],
      "sort-keys": "off",
      "sort-vars": "off",
      "symbol-description": "error",
      "unicode-bom": ["error", "never"],
      "use-isnan": "error",
      "valid-typeof": ["error", { requireStringLiterals: true }],
      "wrap-regex": "off",
    },
  },
  eslintPluginPrettierRecommended,
);
