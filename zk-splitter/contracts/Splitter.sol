// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Splitter {
    address payable public recipient1 = payable(0xC3b97b98cDAe6414D402C33CeE541113B9DD7bC3); // replace with actual
    address payable public recipient2 = payable(0xBe5B4Ee8Be95f7720195918dBcEC3E4B0937b1aa);
    address payable public recipient3 = payable(0xc6Ee69cfD152cD790Cebc301E29e0A55635Eb934);

    receive() external payable {
        uint256 amount = msg.value;
        require(amount > 0, "No ETH sent");

        uint256 share1 = amount * 25 / 100;
        uint256 share2 = amount * 25 / 100;
        uint256 share3 = amount - share1 - share2; // to handle rounding

        (bool sent1, ) = recipient1.call{value: share1}("");
        require(sent1, "Failed to send to recipient1");

        (bool sent2, ) = recipient2.call{value: share2}("");
        require(sent2, "Failed to send to recipient2");

        (bool sent3, ) = recipient3.call{value: share3}("");
        require(sent3, "Failed to send to recipient3");
    }
}
