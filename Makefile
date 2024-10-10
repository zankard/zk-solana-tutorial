CIRCOM=circom
CIRCUITS=./circuits
BUILD=./build

build-circuits:
	$(CIRCOM) $(CIRCUITS)/multiplier.circom --r1cs --wasm --sym -o $(BUILD)
