module.exports = {
  'env': {
    'browser': true,
    'es2021': true,
    'node': true
  },
  'globals': {
    'contextBridge': 'readonly',
    '__SLIMAIR_REQUEST_TIMEOUT_IN_MS': true
  },
  'extends': [
    'eslint:recommended',
    'prettier',
    'plugin:react/recommended'
  ],
  'overrides': [
  ],
  'parser': '@babel/eslint-parser',
  'parserOptions': {
    'babelOptions': {
      'presets': ['@babel/preset-react']
    },
    'ecmaVersion': 'latest',
    'sourceType': 'module'
  },
  'plugins': [
    'react',
    'import',
    'babel'
  ],
  'rules': {
    'eqeqeq': 0,
    'eol-last': 2,
    'no-var': 'error',
    'no-alert': 'error',
    'no-debugger': 'error',
    'no-useless-escape': 'error',
    'space-before-function-paren': [2, { 'anonymous': 'always', 'named': 'never' }],
    'yoda': 'error',
    'arrow-parens': [0, 'always'],
    'arrow-spacing': 2,
    'brace-style': [2, 'stroustrup'],
    'padded-blocks': 0,
    'newline-after-var': 0,
    'spaced-comment': 0,
    'max-len': [2, 100, 4, { 'ignoreUrls': true }],
    'array-bracket-spacing': [2, 'never'],
    'computed-property-spacing': [2, 'never'],
    'no-trailing-spaces': [2, { 'skipBlankLines': true }],
    'no-console': 2,
    'trailingComma': 0,
    'object-curly-spacing': ['error', 'always'],

    'react/jsx-boolean-value': 2,
    'react/jsx-no-undef': 2,
    'react/jsx-sort-prop-types': 0,
    'react/jsx-sort-props': 0,
    'react/no-unescaped-entities': 0,
    'react/jsx-uses-react': 1,
    'react/jsx-uses-vars': 1,
    'react/jsx-curly-spacing': [2, 'never'],
    'react/jsx-indent-props': [2, 2],
    'react/jsx-max-props-per-line': [2, { 'maximum': 3 }],
    'react/jsx-no-bind': 0,
    'react/react-in-jsx-scope': 0,
    'react/jsx-no-duplicate-props': 2,
    'indent': ['error', 2],
    'linebreak-style': [
      'error',
      'unix'
    ],
    'quotes': [
      'error',
      'single'
    ],
    'semi': [
      'error',
      'never'
    ],

    'no-restricted-exports': [2],
    'import/no-unresolved': [2, { 'commonjs': true, 'amd': true }],
    'import/named': 0,
    'import/namespace': 0,
    'import/default': 0,
    'import/export': 2,
    'import/no-named-as-default': 0,
    'import/no-commonjs': 0,
    'import/no-amd': 2,
    'import/imports-first': 2,
    'import/no-duplicates': 2,
    'import/order': ['error',
      {
        'groups': [
          ['builtin', 'external'],
          'internal', 'parent', 'sibling', 'index'
        ],
        'pathGroups': [
          {
            'pattern': '@kobiton/**',
            'group': 'external',
            'position': 'after'
          },
          {
            'pattern': 'react',
            'group': 'builtin',
            'position': 'before'
          },
        ],
        'pathGroupsExcludedImportTypes': ['builtin'],
        'alphabetize': {
          'order': 'asc', /* sort in ascending order. Options: ['ignore', 'asc', 'desc'] */
          'caseInsensitive': true /* ignore case. Options: [true, false] */
        }
      }
    ]
  },
}
