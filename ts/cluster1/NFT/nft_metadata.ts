import { Keypair, Connection, Commitment } from "@solana/web3.js";
import wallet from "../wba-wallet.json"
import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet))

const commitment: Commitment = "confirmed";
const connection= new Connection("https://api.devnet.solana.com");

const metaplex = Metaplex.make(connection).use(keypairIdentity(keypair)).use(bundlrStorage({
    
    address: 'https://devnet.bundlr.network',
    providerUrl: "https://api.devnet.solana.com",
    timeout: 60_000 
}));

async function uploadMetadata(){
    try{
        const uri = await metaplex.nfts().uploadMetadata({
            
            name: "Eleonora's rug",
            symbol: "Elnr",
            description: "",
            image: "https://arweave.net/MyKxBNXyOU0tJrzCs6TF-YZku3SchFmNdsdOfuJ9EPc",
            
            attributes: [
                {trait_type: 'Feature', value: 'Vaporwave Pink'},
                {trait_type: 'Style', value: 'Pixelated'},
                {trait_type: 'Background', value: 'Minty Green'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://arweave.net/MyKxBNXyOU0tJrzCs6TF-YZku3SchFmNdsdOfuJ9EPc",
                    },
                ]
            },
            creators: [
                {
                  address: keypair.publicKey.toBase58(),
                  share: 100
                }
            ] 
        })

        console.log(`URI = ${uri.uri}`)

        //URI = https://arweave.net/fFmAvwO0dVG1SnXr4CQqhft85hEEFmmKikXsKrjTG1g
        
    }catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
}

uploadMetadata()