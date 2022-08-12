// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";

contract TestBridgeToken is
    Initializable,
    ERC20Upgradeable,
    AccessControlUpgradeable,
    PausableUpgradeable
{

    string private _name;
    string private _symbol;
    uint8 private _decimals;

    uint64 _metadataLastUpdated;

    bytes32 public constant PAUSE_ROLE = keccak256("PAUSE_ROLE");

    function initialize(
        string memory name_,
        string memory symbol_,
        uint8 decimals_
    ) external initializer {

        __ERC20_init(_name, _symbol);
        __AccessControl_init();

        _setupRole(DEFAULT_ADMIN_ROLE, _msgSender()); //should be sender or controller?
        _setupRole(PAUSE_ROLE, _msgSender());
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
    }

    function pause() external onlyRole(PAUSE_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(PAUSE_ROLE) {
        _unpause();
    }

    function setMetadata(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        uint64 blockHeight_
    ) public onlyRole(DEFAULT_ADMIN_ROLE) {
        _metadataLastUpdated = blockHeight_;
        _name = name_;
        _symbol = symbol_;
        _decimals = decimals_;
    }

    function mint(address beneficiary, uint256 amount)
        public
        onlyRole(DEFAULT_ADMIN_ROLE)
        whenNotPaused
    {
        _mint(beneficiary, amount);
    }

    function burn(address act, uint256 amount)
        public
        onlyRole(DEFAULT_ADMIN_ROLE)
        whenNotPaused
    {
        _burn(act, amount);
    }

    function name() public view virtual override returns (string memory) {
        return _name;
    }

    function symbol() public view virtual override returns (string memory) {
        return _symbol;
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    function metadataLastUpdated() public view virtual returns (uint64) {
        return _metadataLastUpdated;
    }

    function returnTestString() public pure returns (string memory) {
        return "test";
    }
}
