package bishan.grapher3d.request;

public enum RequestType {
	GET, POST, PUT, DELETE, HEAD, OTHER;

	public boolean is(String reqMethod) {
		return this.toString().equalsIgnoreCase(reqMethod);
	}
}
