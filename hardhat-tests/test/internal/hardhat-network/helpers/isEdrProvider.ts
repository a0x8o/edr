/* eslint-disable dot-notation,@typescript-eslint/dot-notation */
import { EdrProviderWrapper } from "hardhat/internal/hardhat-network/provider/provider";

export function isEdrProvider(provider: any): boolean {
  return (
    provider instanceof EdrProviderWrapper ||
    provider["_provider"] instanceof EdrProviderWrapper
  );
}
