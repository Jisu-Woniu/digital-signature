import pluginVue from "eslint-plugin-vue";
import vueTsEslintConfig from "@vue/eslint-config-typescript";
import vuePrettierEslintConfig from "@vue/eslint-config-prettier";

export default [
  ...pluginVue.configs["flat/recommended"],
  ...vueTsEslintConfig(),
  vuePrettierEslintConfig,
];
