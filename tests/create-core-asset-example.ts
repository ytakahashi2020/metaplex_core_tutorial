import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateCoreAssetExample } from "../target/types/create_core_asset_example";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";

describe("create-core-asset-example", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.Wallet.local();
  console.log(wallet.publicKey);
  const program = anchor.workspace
    .CreateCoreAssetExample as Program<CreateCoreAssetExample>;

  let asset = Keypair.generate();

  it("Create Asset", async () => {
    let createAssetArgs = {
      name: "My Asset",
      uri: "https://example.com/my-asset.json",
    };

    const createAssetTx = await program.methods
      .createCoreAsset(createAssetArgs)
      .accounts({
        asset: asset.publicKey,
        collection: null,
        authority: null,
        payer: wallet.publicKey,
        owner: null,
        updateAuthority: null,
        systemProgram: SystemProgram.programId,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([asset, wallet.payer])
      .rpc();

    console.log(createAssetTx);

    console.log(
      `https://core.metaplex.com/explorer/${asset.publicKey}?env=devnet`
    );
  });
});
