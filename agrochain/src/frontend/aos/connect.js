// Lightweight Keplr + CosmJS connector for Andromeda aOS
export async function connectAOSWithKeplr({ chainId, rpc, prefix }) {
  if (!window.keplr) throw new Error('Keplr not found');

  await window.keplr.enable(chainId);
  const offlineSigner = window.keplr.getOfflineSigner(chainId);
  const accounts = await offlineSigner.getAccounts();
  const signerAddress = accounts[0]?.address;

  const { SigningCosmWasmClient } = await import('@cosmjs/cosmwasm-stargate');
  const gasPrice = (await import('@cosmjs/stargate')).GasPrice.fromString('0.025uandr');
  const client = await SigningCosmWasmClient.connectWithSigner(rpc, offlineSigner, { gasPrice });

  return { client, signerAddress };
}


