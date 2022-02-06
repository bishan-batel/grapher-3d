package bishan.grapher3d.request;

import bishan.grapher3d.handlers.auth.SessionManager;
import com.sun.net.httpserver.HttpExchange;

import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;

import static java.lang.String.format;

public class RequestInfo {
	public static final String DELIMITER = ": ";

	private final HttpExchange ex;
	private final String route;
	private final String[] routeSections;
	private final RequestType reqType;
	private final OutputStream responseBody;
	private final String sessionId;
	// TODO ammend
	private final String cookies;

	public RequestInfo(HttpExchange ex) {
		this.ex = ex;
		route = ex.getRequestURI().getPath();
		routeSections = route.substring(1).split("/");

		responseBody = ex.getResponseBody();
		cookies = ex.getRequestHeaders().getFirst("Cookie");
		sessionId = getCookie("token");

		RequestType type;
		try {
			type = RequestType.valueOf(ex.getRequestMethod());
		} catch (IllegalArgumentException e) {
			type = RequestType.OTHER;
		}
		reqType = type;
	}

	public boolean isVerb(RequestType reqType) {
		return this.reqType.is(ex.getRequestMethod());
	}

	@Override
	public String toString() {
		return format(
			"%s Request at route \"%s\" with session of \"%s\"",
			reqType.toString(),
			route,
			sessionId
		);
	}

	// Getters & Setters
	public HttpExchange getEx() {
		return ex;
	}

	public String getRoute() {
		return route;
	}

	public String[] getRouteSections() {
		return routeSections;
	}

	public String getRouteSection(int i) {
		try {
			return routeSections[i];
		} catch (Exception e) {
			return null;
		}
	}

	public RequestType getReqType() {
		return reqType;
	}

	public OutputStream getResponseBody() {
		return responseBody;
	}

	public String getSessionId() {
		return sessionId;
	}

	public String getCookie(String name) {
		// first half of what cookie would be
		var nameEq = name + "=";

		// splits cookiues by ;
		String[] cookiesSplit = cookies.split(";");

		// for each cookie
		for (String cookie : cookiesSplit) {

			// trim whitespace
			while (cookie.charAt(0) == ' ')
				cookie = cookie.substring(1);

			// if beginning equals nameEq then return substring val
			if (cookie.indexOf(nameEq) == 0)
				return cookie.substring(nameEq.length());
		}

		// no cookie found
		return null;
	}

	public Map<String, String> getRequestBody() throws IOException {
		var map = new HashMap<String, String>();
		// if request type is not YAML then assume wrong format


		// gets request body as string
		byte[] bodyBytes = ex.getRequestBody().readAllBytes();
		var body = new String(bodyBytes, StandardCharsets.UTF_8);

		// for all lines in the string try to split it into key value string pairs
		for (String line : body.split("\n")) {

			// if line does not contain delimiter then assume wrong format
			if (!line.contains(DELIMITER)) continue;

			// get key on left side of delimeter & put in map
			String key = line.split(DELIMITER)[0];
			map.put(key, line.substring(key.length() + DELIMITER.length()));
		}

		return map;
	}
}
