package bishan.grapher3d.handlers.auth;

import java.math.BigInteger;
import java.util.HashMap;
import java.util.Map;
import java.util.Random;

/**
 * This class is used to manage cookie tokens from the client
 */
public class SessionManager
{

  public static final int TOKEN_LENGTH = 32;
  private final Map<String, Integer> sessions;

  public SessionManager()
  {
    sessions = new HashMap<>();
  }

  public String createToken(int uid)
  {
    String token = null;

    // crates random num generator
    var rng = new Random();

    // empty buffer for random bytes
    var bytes = new byte[TOKEN_LENGTH];

    // until token is not null & generated token is not a duplicate
    while (token == null || sessions.containsKey(token))
    {
      // fill buffer with random bytes
      rng.nextBytes(bytes);

      // convert to hex
      var hexBuilder = new StringBuilder(new BigInteger(bytes).toString(16));

      // pads hex to keep length
      while (hexBuilder.length() < TOKEN_LENGTH)
      {
        hexBuilder.insert(0, '0');
      }

      // generate token
      token = hexBuilder.toString();
    }

    // places token in session store
    sessions.put(token, uid);
    return token;
  }

  public void clear(String tok)
  {
    sessions.remove(tok, sessions.get(tok));
  }

  public void clear()
  {
    sessions.clear();
  }

  // Setters and Getters
  public Integer getUID(String tok)
  {
    return sessions.get(tok);
  }

  public boolean isValidToken(String tok)
  {
    return sessions.containsKey(tok) && getUID(tok) != null;
  }
}
