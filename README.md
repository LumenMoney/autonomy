Add a terra account using the following command

    terrad keys add spatial --recover
Upon asking enter the following mnemonic

    spatial forest elevator battle also spoon fun skirt flight initial nasty transfer glory palm drama gossip remove fan joke shove label dune debate quick

Now we have to increase allowance on the CW20 token which we want to transfer to the contract. Following is an example where the CW20 contract address is. 

> terra10ce4c85lv3y5euz6wtcj7r2r549n4hm837t6m3

Replace it with the CW20 token address you wish to transfer

    terrad tx wasm execute terra10ce4c85lv3y5euz6wtcj7r2r549n4hm837t6m3 '{"increase_allowance": {"spender":"terra19zwvfln347ypht3svhswcc66lx66g8tugl8ua5","amount":"4000000000000"}}' --node https://bombay.stakesystems.io:2053 --from spatial --fees=1000000uluna --gas=auto --broadcast-mode=block --chain-id bombay-12

After increasing the allowance, we can transfer token to the contract which is 

> terra19zwvfln347ypht3svhswcc66lx66g8tugl8ua5

    terrad tx wasm execute terra19zwvfln347ypht3svhswcc66lx66g8tugl8ua5 '{"deposit": {"contract_addr":"terra10ce4c85lv3y5euz6wtcj7r2r549n4hm837t6m3","amount":"140"}}' --node https://bombay.stakesystems.io:2053 --from spatial --fees=1000000uluna --gas=auto --broadcast-mode=block --chain-id bombay-12

Once we have transferred CW20 token to contract we can withdraw with following command

    terrad tx wasm execute terra19zwvfln347ypht3svhswcc66lx66g8tugl8ua5 '{"withdraw": {"contract_addr":"terra10ce4c85lv3y5euz6wtcj7r2r549n4hm837t6m3","amount":"40"}}' --node https://bombay.stakesystems.io:2053 --from spatial --fees=1000000uluna --gas=auto --broadcast-mode=block --chain-id bombay-12



