package com.github.stepancheg.rustide.client.util;

import com.google.protobuf.Message;

import java.util.Collection;

/**
 * @author Stepan Koltsov
 */
public class ProtobufUnion {
    public static Message unionMember(Message union) {
        Collection<Object> fields = union.getAllFields().values();
        if (fields.size() == 0) {
            throw new IllegalArgumentException("field not set");
        } else if (fields.size() > 1) {
            throw new IllegalArgumentException("not a union: too many fields set: " + fields.size());
        } else {
            return (Message) fields.iterator().next();
        }
    }

    public static <T extends Message> T unionMember(Message union, Class<T> memberType) {
        return memberType.cast(unionMember(union));
    }
}
