package bishan.grapher3d.handlers.auth;

import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Random;

import static java.lang.String.format;

/**
 * Wrapper to create unique hashes for a password with a salt
 * Also used to compare multiple passwords
 */
public class SaltedPassword {
	// Constants
	public static final String HASH_ALGORITHM = "SHA-256";
	public static final int HASH_BIT_LENGTH = 256;
	public static final int SALT_SIZE = 32;

	// Instance variables
	private String hash;
	private final String salt;

	// Constructors
	public SaltedPassword(String password) {
		salt = getRandomSalt();
		setHash(password);
	}

	public SaltedPassword(String hash, String salt) {
		this.salt = salt;
		this.hash = hash;
	}

	@Override
	public boolean equals(Object obj) {
		// if other obj is null then not equal
		if (obj == null) return false;

		// if other object is not same type then not equal
		if (!(obj instanceof SaltedPassword)) return false;

		// casts to type
		var rhs = (SaltedPassword) obj;

		// 2 salted passwords are equal if their calculated hashes are the same
		return rhs.hash.equals(this.hash);
	}

	@Override
	public String toString() {
		return format("Hash: [%s]\nSalt: [%s]", hash, salt);
	}

	// Setters and getters
	public void setHash(String password) {
		try {
			// creates hash digester
			MessageDigest msgDigest = MessageDigest.getInstance(HASH_ALGORITHM);

			// gets 'food' to feed into digester
			byte[] food = (password + salt).getBytes();

			// hashes into byte array
			byte[] digested = msgDigest.digest(food);

			// stringifies hash to HEX
			var hex = new StringBuilder(new BigInteger(digested)
				.abs()
				.toString(16)
			);

			// keeps length of string constant
			while (hex.length() < (HASH_BIT_LENGTH / Byte.SIZE))
				hex.insert(0, '0');

			// builds string
			hash = hex.toString();

			/*
			 bruh whoever made java.security made it so that when you get a digest
			 you need to handle a 'NoSuchAlgorithmException' in case the algorithm
			 you specify (which is a STRING)
			 */
		} catch (NoSuchAlgorithmException nsa) {
			nsa.printStackTrace();
		}
	}

	public String getHash() {
		return hash;
	}

	public static String getRandomSalt() {
		// Creates empty buffer for salt
		var salt = new byte[SALT_SIZE];

		// Creates RNG
		var rng = new Random();

		// Fills buffer with random bytes
		rng.nextBytes(salt);

		// converts bytes to hex
		var hex = new StringBuilder(new BigInteger(salt)
			.abs()
			.toString(16)
		);

		// pads 0's to keep length constant
		while (hex.length() < SALT_SIZE)
			hex.insert(0, '0');

		return hex.toString();
	}

	public String getSalt() {
		return salt;
	}

	// Tests
	public static void main(String[] args) {
		// test for salt
		var salted = new SaltedPassword("password123");
		System.out.println(salted);

		String salt = salted.getSalt();

		for (int i = 0; i < salt.length(); i++) {
			System.out.print((byte) salt.charAt(i) + ", ");
		}

		System.out.println();
	}
}
