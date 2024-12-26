// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ArrayExample {
    // Declare a dynamic array of unsigned integers
    uint[] public numbers;

    // Add a number to the array
    function addNumber(uint _number) public {
        numbers.push(_number);
    }

    // Get the number at a specific index
    function getNumber(uint _index) public view returns (uint) {
        require(_index < numbers.length, "Index out of bounds");
        return numbers[_index];
    }

    // Update the number at a specific index
    function updateNumber(uint _index, uint _newNumber) public {
        require(_index < numbers.length, "Index out of bounds");
        numbers[_index] = _newNumber;
    }

    // Remove the last number in the array
    function removeLastNumber() public {
        require(numbers.length > 0, "Array is empty");
        numbers.pop();
    }

    // Get the length of the array
    function getArrayLength() public view returns (uint) {
        return numbers.length;
    }

    // Get all the numbers in the array
    function getAllNumbers() public view returns (uint[] memory) {
        return numbers;
    }
}
