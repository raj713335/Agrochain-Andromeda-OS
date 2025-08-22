import React, { useReducer } from "react";
import { createContext } from "react";
import { SET_ACCOUNT, SET_ACCOUNT_TYPE, SET_BALANCE, SET_LOADING, SET_MARKETPLACE, SET_NFT, SET_AOS_CLIENT, SET_AOS_ADDRESS, SET_CARBON_ADO_ADDRESS } from "./nftActions";
import { connectAOSWithKeplr } from "../aos/connect";
import nftReducer from "./nftReducer";

export const NftContext = createContext();

const NftProvider = ({ children }) => {
    const initialState = {
        marketplace: {},
        nft: {},
        account: '',
        balance: 0,
        isLoading: false,
        accountType: false,
        // aOS (Cosmos) state
        aosClient: null,
        aosAddress: '',
        aosNetwork: 'andromeda',
        carbonAdoAddress: ''
    };
    const [state, dispatch] = useReducer(nftReducer, initialState);
    const setAccount = (payload) => {
        dispatch({ type: SET_ACCOUNT, payload })
    }
    const setMarketplace = (payload) => {
        dispatch({ type: SET_MARKETPLACE, payload })
    }
    const setNFT = (payload) => {
        dispatch({ type: SET_NFT, payload })
    }
    const setBalance = (payload) => {
        dispatch({ type: SET_BALANCE, payload })
    }
    const setIsLoading = (payload) => {
        dispatch({ type: SET_LOADING, payload })
    }
    const setAccountType = (payload) => {
        dispatch({ type: SET_ACCOUNT_TYPE, payload })
    }
    const setAosClient = (payload) => {
        dispatch({ type: SET_AOS_CLIENT, payload })
    }
    const setAosAddress = (payload) => {
        dispatch({ type: SET_AOS_ADDRESS, payload })
    }
    const setCarbonAdoAddress = (payload) => {
        dispatch({ type: SET_CARBON_ADO_ADDRESS, payload })
    }

    const connectAOS = async ({ chainId = 'andromeda-1', rpc = 'https://rpc-andromeda-1.takeshi.team', prefix = 'andr', carbonAdoAddress = '' } = {}) => {
        const { client, signerAddress } = await connectAOSWithKeplr({ chainId, rpc, prefix });
        setAosClient({ client, signerAddress });
        setAosAddress(signerAddress);
        if (carbonAdoAddress) setCarbonAdoAddress(carbonAdoAddress);
        return { client, signerAddress };
    };
    return (
        <NftContext.Provider value={{
            account: state.account,
            marketplace: state.marketplace,
            nft: state.nft,
            balance: state.balance,
            isLoading: state.isLoading,
            accountType: state.accountType,
            aosClient: state.aosClient,
            aosAddress: state.aosAddress,
            aosNetwork: state.aosNetwork,
            carbonAdoAddress: state.carbonAdoAddress,
            setAccount,
            setMarketplace,
            setNFT,
            setBalance,
            setIsLoading,
            setAccountType,
            setAosClient,
            setAosAddress,
            setCarbonAdoAddress,
            connectAOS
        }}>
            {children}
        </NftContext.Provider>
    )
};

export default NftProvider;
