package bishan.grapher3d.handlers.auth;

import bishan.grapher3d.MainServer;
import bishan.grapher3d.database.SQLDb;
import bishan.grapher3d.database.SetupDB;
import bishan.grapher3d.request.RequestInfo;
import com.sun.net.httpserver.HttpExchange;

import java.io.IOException;
import java.io.OutputStream;
import java.sql.SQLException;
import java.util.Arrays;
import java.util.Map;
import java.util.UUID;

import static java.lang.String.format;

public class AuthHandler {

	private MainServer main;
	private SessionManager sessionManager;

	public AuthHandler(MainServer main) {
		this.main = main;
		sessionManager = new SessionManager();
	}

	public void handle(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		String subRoute = req.getRouteSection(1);

		switch (subRoute) {
			case "login": {
				login(req);
				return;
			}

			case "validate": {
				validate(req);
				return;
			}

			case "logout": {
				logout(req);
				return;
			}

			case "register": {
				byte[] msg = "Success".getBytes();
				OutputStream body = ex.getResponseBody();
				Map<String, String> reqBody = req.getRequestBody();

				try {
					// try to register
					String email = reqBody.get("email");
					String password = reqBody.get("password");
					register(email, password);
					ex.sendResponseHeaders(200, msg.length);
				} catch (SQLException e) {
					// if failed send server error
					msg = "Internal Server Error".getBytes();
					ex.sendResponseHeaders(500, msg.length);
					e.printStackTrace();
				} catch (IllegalArgumentException e) {
					// if user exists send 409 conflict
					msg = "User already exists".getBytes();
					ex.sendResponseHeaders(409, msg.length);
				}
				body.write(msg);
				body.close();
				return;
			}

			default:
				break;
		}

		byte[] err = format("Invalid route '%s'", subRoute).getBytes();
		OutputStream body = ex.getResponseBody();
		ex.sendResponseHeaders(404, err.length);
		body.write(err);
		body.close();
	}

	public void validate(RequestInfo info) throws IOException {
		String sessionId = info.getSessionId();
		HttpExchange ex = info.getEx();
		OutputStream body = info.getResponseBody();

		// if not valid session return 404 error
		if (!sessionManager.isValidToken(sessionId)) {
			ex.sendResponseHeaders(404, 0);
			body.close();
			return;
		}

		var uid = (int) sessionManager.getUID(sessionId);

		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			// get user row
			var userRow = db.selectWhere(
				SetupDB.USERS_TABLE,
				"id=?",
				uid
			)[0];

			// gets email
			var email = userRow[1];

			// creates JSON format to send back
			var json = ("{\"email\":\"" + email + "\"}").getBytes();

			// turns json into bytes
			// sends OK response
			ex.sendResponseHeaders(200, json.length);
			body.write(json);
			body.close();
		} catch (Exception e) {
			// Send internal server error
			var msg = "Internal server error".getBytes();
			ex.sendResponseHeaders(500, msg.length);
			body.write(msg);
			body.close();
			e.printStackTrace();
		}
	}

	public void register(String email, String pass) throws SQLException, IllegalArgumentException {
		// connections to database
		var db = new SQLDb(SetupDB.DB_NAME);

		// tries to find any user that already have the same email
		var existingUsersWithEmail = db.selectWhere(
			SetupDB.USERS_TABLE,
			"email=?",
			email
		);

		// if any users exist with same email throw exception
		if (existingUsersWithEmail.length > 0) {
			throw new IllegalArgumentException("Duplicate email");
		}

		// Salt & Hash password
		var salted = new SaltedPassword(pass);

		int uid = 0;
		String[][] duplicateUids;
		var isUidDuplicated = true;

		// repeat until generated UID is not a duplicate
		while (isUidDuplicated) {
			uid = UUID.randomUUID().hashCode();

			// get all users in table with same UID
			duplicateUids = db.selectWhere(SetupDB.USERS_TABLE, "id=?", uid);

			// isUiDuplicated true if there are any duplicates found
			isUidDuplicated = duplicateUids.length > 0;
		}

		// creates new user
		db.insert(
			SetupDB.USERS_TABLE,
			uid,
			email,
			salted.getHash(),
			salted.getSalt()
		);
	}

	public void login(RequestInfo info) throws IOException {
		HttpExchange ex = info.getEx();
		OutputStream body = info.getResponseBody();

		// Request Information
		Map<String, String> req = info.getRequestBody();
		String email = req.get("email");
		String password = req.get("password");

		// connect to DB
		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			// attempts to find a user with same email as requested
			String[][] usersResult = db.selectWhere(
				SetupDB.USERS_TABLE,
				"email=?",
				email
			);

			// if user does not exist
			if (usersResult.length < 1) {
				byte[] msg = "User does not exist".getBytes();
				ex.sendResponseHeaders(404, msg.length);
				body.write(msg);
				body.close();
				return;
			}

			// parses row for stored password hash & it's salt
			int uid = Integer.parseInt(usersResult[0][0]);

			// retrieves salted password from database for user
			SaltedPassword correctPassword = passwordDetailsFor(uid);

			// creates salted password construct from incoming password
			var incomingPassword = new SaltedPassword("", correctPassword.getSalt());
			incomingPassword.setHash(password);

			// if they do not equal to each other that means the password is valid
			if (!correctPassword.equals(incomingPassword)) {
				// send unauthorized status code
				byte[] msg = "Invalid Password".getBytes();
				ex.sendResponseHeaders(401, msg.length);
				body.write(msg);
				body.close();
				return;
			}

			// If password valid then create new session
			byte[] tok = sessionManager.createToken(uid).getBytes();

			// Send session token back
			ex.sendResponseHeaders(200, tok.length);
			body.write(tok);
			body.close();

		} catch (SQLException | NumberFormatException e) {
			// Send internal server error
			byte[] msg = "Internal server error".getBytes();
			ex.sendResponseHeaders(500, msg.length);
			body.write(msg);
			body.close();

			e.printStackTrace();
		}
	}

	public void logout(RequestInfo info) throws IOException {
		HttpExchange ex = info.getEx();
		OutputStream body = info.getResponseBody();
		String sessionId = info.getSessionId();
		byte[] msg;

		if (!sessionManager.isValidToken(sessionId)) {
			// Send 409 Conflict Response back
			msg = "Not logged in".getBytes();
			ex.sendResponseHeaders(409, msg.length);
		} else {
			// Delete session
			sessionManager.clear(sessionId);

			// Send OK response
			msg = "Logged out successfully".getBytes();
			ex.sendResponseHeaders(200, msg.length);
		}

		// writes message and closes
		body.write(msg);
		body.close();
	}

	public SaltedPassword passwordDetailsFor(int uid) throws SQLException {
		var db = new SQLDb(SetupDB.DB_NAME);
		String[][] results = db.selectWhere(
			SetupDB.USERS_TABLE,
			"id=?",
			uid
		);
		db.close();

		if (results == null || results.length == 0) {
			throw new SQLException(format("UID '%d' does not exist", uid));
		}

		String[] user = results[0];
		String hash = user[2], salt = user[3];

		return new SaltedPassword(hash, salt);
	}

	// Setters and Getters
	public MainServer getMain() {
		return main;
	}

	public boolean isAuthorizedForGraph(
		String session,
		String graphName
	) throws SQLException, IllegalArgumentException {
		// exit it invalid session ID
		if (!sessionManager.isValidToken(session)) {
			return false;
		}

		var uid = (int) sessionManager.getUID(session);
		var db = new SQLDb(SetupDB.DB_NAME);

		// retrieves graph
		String[][] graphResult = db.selectWhere(
			SetupDB.GRAPHS_TABLE,
			"graphName=? AND owner=?",
			graphName,
			uid
		);
		db.close(); // close DB conn

		return graphResult.length > 0;
	}

	public SessionManager getSessionManager() {
		return sessionManager;
	}
}
