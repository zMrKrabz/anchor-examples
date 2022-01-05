import 'mocha';
import { expect } from 'chai';
import * as anchor from '@project-serum/anchor';

const SystemProgram = anchor.web3.SystemProgram;

describe('roulette', () => {
	const provider = anchor.Provider.local();
	anchor.setProvider(provider);

	const gameAccount = anchor.web3.Keypair.generate();

	const program = anchor.workspace.Roulette;
	it('initializes account', async () => {
		await program.rpc.initialize(new anchor.BN(0), {
			accounts: {
				game: gameAccount,
				user: provider.wallet.publicKey,
				systemProgram: SystemProgram,
			},
			signers: [gameAccount],
		});

		const game = await program.account.gameAccount.fetch(gameAccount.publicKey);
		expect(game.counter).eq(new anchor.BN(0));
	});

	it('increase account counter', async () => {
		const game = await program.account.gameAccount.fetch(gameAccount.publicKey);
		await program.rpc.increase({
			accounts: {
				game: gameAccount,
				authority: provider.wallet.publicKey,
			}
		});

		expect(game.counter).eq(new anchor.BN(1));
	});

	it('decrease account counter', async () => {
		const game = await program.account.gameAccount.fetch(gameAccount.publicKey);
		await program.rpc.decrease({
			accounts: {
				game: gameAccount,
				authority: provider.wallet.publicKey,
			}
		})
		expect(game.counter).eq(new anchor.BN(0));
	})
});