import { useState } from 'react';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, Provider, web3 } from '@project-serum/anchor';
import idl from './idl.json'; // Your IDL file

const { SystemProgram, Keypair } = web3;
const programID = new PublicKey(idl.metadata.address);
const network = clusterApiUrl('devnet');
const opts = {
    preflightCommitment: "processed"
};

const App = () => {
    const [walletAddress, setWalletAddress] = useState(null);

    const getProvider = () => {
        const connection = new Connection(network, opts.preflightCommitment);
        const provider = new Provider(connection, window.solana, opts.preflightCommitment);
        return provider;
    }

    const depositUSDC = async (amount) => {
        const provider = getProvider();
        const program = new Program(idl, programID, provider);

        try {
            await program.rpc.depositUsdc(new web3.BN(amount), {
                accounts: {
                    vault: vaultPublicKey,
                    userAccount: userTokenAccountPublicKey,
                    user: provider.wallet.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                },
            });
        } catch (err) {
            console.error("Transaction error: ", err);
        }
    }

    return (
        <div>
            <button onClick={() => depositUSDC(100)}>Deposit 100 USDC</button>
        </div>
    );
};

export default App;
