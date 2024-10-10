CIRCOM=circom
SNARKJS=snarkjs
CIRCUITS=./circuits
BUILD=./build
SETUP_DIR=$(BUILD)/setup

build-circuits:
	$(CIRCOM) $(CIRCUITS)/multiplier.circom --r1cs --wasm --sym -o $(BUILD)/circuits

prepare-phase1:
	$(SNARKJS) powersoftau new bn128 12 $(SETUP_DIR)/pot12_0000.ptau -v

contribute-phase1:
	$(SNARKJS) powersoftau contribute $(SETUP_DIR)/pot12_0000.ptau $(SETUP_DIR)/pot12_0001.ptau --name="First contribution" -v -e="random text"

beacon:
	$(SNARKJS) powersoftau beacon $(SETUP_DIR)/pot12_0001.ptau $(SETUP_DIR)/pot12_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d2222 10 -n="Final Beacon"

prepare-phase2:
	$(SNARKJS) powersoftau prepare phase2 $(SETUP_DIR)/pot12_beacon.ptau $(SETUP_DIR)/pot12_final.ptau -v

verify-ptau:
	$(SNARKJS) powersoftau verify $(SETUP_DIR)/pot12_final.ptau 

final-zkey:
	$(SNARKJS) zkey new $(BUILD)/circuits/multiplier.r1cs $(SETUP_DIR)/pot12_final.ptau $(BUILD)/multiplier_0000.zkey
	echo "some random text" | $(SNARKJS) zkey contribute $(BUILD)/multiplier_0000.zkey $(BUILD)/multiplier_final.zkey --name="1st Contributor" -v -e="more random text"
	$(SNARKJS) zkey export verificationkey $(BUILD)/multiplier_final.zkey $(BUILD)/verification_key.json
	rm $(BUILD)/multiplier_0000.zkey

