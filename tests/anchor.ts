import * as anchor from "@coral-xyz/anchor";
import type { ImprovedAssetManagementVault } from "../target/types/improved_asset_management_vault";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ImprovedAssetManagementVault as anchor.Program<ImprovedAssetManagementVault>;
  
  it("initialize", async () => {
  });
});
