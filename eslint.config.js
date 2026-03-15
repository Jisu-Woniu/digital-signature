import pluginVue from "eslint-plugin-vue";
import { vueTsConfigs } from "@vue/eslint-config-typescript";
import { defineConfigWithVueTs } from "@vue/eslint-config-typescript";
import vuePrettierEslintConfig from "@vue/eslint-config-prettier";

export default defineConfigWithVueTs(
  pluginVue.configs["flat/recommended"],
  vueTsConfigs.recommendedTypeChecked,
  vuePrettierEslintConfig,
  {
    rules: {
      "@typescript-eslint/no-misused-promises": [
        "error",
        {
          checksVoidReturn: {
            arguments: false,
          },
        },
      ],
    },
  },
);
