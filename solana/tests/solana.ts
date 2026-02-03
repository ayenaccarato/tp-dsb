import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solana } from "../target/types/solana";
import { expect } from "chai";

describe("solana - libros", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solana as Program<Solana>;
  const user = provider.wallet;

  const id = new anchor.BN(1);

  const [libroPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("libro"), id.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  it("Registra un libro", async () => {
    await program.methods
      .registrarLibro(id, "Libro Solana", "Descripcion")
      .accounts({
        libro: libroPda,
        authority: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const libro = await program.account.libro.fetch(libroPda);

    expect(libro.id.toNumber()).to.equal(1);
    expect(libro.titulo).to.equal("Libro Solana");
    expect(libro.descripcion).to.equal("Descripcion");
    expect(libro.propietario.toBase58()).to.equal(
      user.publicKey.toBase58()
    );
  });

  it("Actualiza el libro", async () => {
    await program.methods
      .actualizarLibro("Nuevo titulo", "Nueva descripcion")
      .accounts({
        libro: libroPda,
        authority: user.publicKey,
      })
      .rpc();

    const libro = await program.account.libro.fetch(libroPda);

    expect(libro.titulo).to.equal("Nuevo titulo");
    expect(libro.descripcion).to.equal("Nueva descripcion");
  });
});
