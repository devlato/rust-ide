package com.github.stepancheg.rustide.client.util;

/**
 * @author Stepan Koltsov
 */
public class Either<A, B> {
    private final A left;
    private final B right;
    private final boolean isLeft;

    public Either(A left, B right, boolean isLeft) {
        this.left = left;
        this.right = right;
        this.isLeft = isLeft;
    }

    public static <A, B> Either<A, B> left(A left) {
        return new Either<A, B>(left, null, true);
    }

    public static <A, B> Either<A, B> right(B right) {
        return new Either<A, B>(null, right, false);
    }

    public A getLeft() {
        if (!isLeft) {
            throw new IllegalStateException();
        }
        return left;
    }

    public B getRight() {
        if (isLeft) {
            throw new IllegalStateException();
        }
        return right;
    }

    public boolean isLeft() {
        return isLeft;
    }
}
