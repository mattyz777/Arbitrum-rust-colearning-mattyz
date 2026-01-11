use alloy::sol;

sol! {
    #[sol(rpc)]
    #[derive(Debug)]
    interface IERC20 {
        function name() external view returns (string memory);
        function symbol() external view returns (string memory);
        function decimals() external view returns (uint8);
        function balanceOf(address account) external view returns (uint256);
    }
}