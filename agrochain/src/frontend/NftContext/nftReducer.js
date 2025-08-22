import { SET_ACCOUNT, SET_ACCOUNT_TYPE, SET_BALANCE, SET_LOADING, SET_MARKETPLACE, SET_NFT, SET_AOS_CLIENT, SET_AOS_ADDRESS, SET_CARBON_ADO_ADDRESS } from "./nftActions";

const nftReducer = (state, action) => {
    switch (action.type) {
        case SET_ACCOUNT:
            return { ...state, account: action.payload };
        case SET_MARKETPLACE:
            return { ...state, marketplace: action.payload };
        case SET_NFT:
            return { ...state, nft: action.payload };
        case SET_BALANCE:
            return { ...state, balance: action.payload };
        case SET_LOADING:
            return { ...state, isLoading: action.payload };
        case SET_ACCOUNT_TYPE:
            return { ...state, accountType: action.payload };
        case SET_AOS_CLIENT:
            return { ...state, aosClient: action.payload };
        case SET_AOS_ADDRESS:
            return { ...state, aosAddress: action.payload };
        case SET_CARBON_ADO_ADDRESS:
            return { ...state, carbonAdoAddress: action.payload };
        default:
            return state;
    }
};

export default nftReducer;
