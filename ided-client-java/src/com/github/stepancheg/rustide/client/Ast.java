package com.github.stepancheg.rustide.client;

import com.github.stepancheg.rustide.client.proto.IdedProtos;
import com.github.stepancheg.rustide.client.util.Either;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

/**
 * @author Stepan Koltsov
 */
public class Ast {
    private final IdedProtos.FileSpan span;
    private final List<Ast> children;

    public Ast(IdedProtos.FileSpan span, List<Ast> children) {
        this.children = children;
        this.span = span;
    }

    public static Ast fromLazy(IdedProtos.LazyAst lazyAst) {
        Iterator<IdedProtos.NodeMarker> iterator = lazyAst.getMarkersList().iterator();
        if (!iterator.hasNext()) {
            throw new IllegalArgumentException("must at least have root");
        }
        Either<Ast, IdedProtos.NodeMarker> r = fromLazyIterator(iterator);
        if (iterator.hasNext()) {
            throw new IllegalArgumentException();
        }
        return r.getLeft();
    }

    private static Either<Ast, IdedProtos.NodeMarker> fromLazyIterator(Iterator<IdedProtos.NodeMarker> iterator) {
        List<Ast> children = new ArrayList<Ast>();
        IdedProtos.NodeMarker first = iterator.next();
        if (!first.getOpen()) {
            return Either.right(first);
        }
        for (;;) {
            Either<Ast, IdedProtos.NodeMarker> next = fromLazyIterator(iterator);
            if (next.isLeft()) {
                children.add(next.getLeft());
            } else {
                if (first.getNodeType() != next.getRight().getNodeType()) {
                    throw new IllegalStateException();
                }
                IdedProtos.FileSpan.Builder fileSpanBuilder = IdedProtos.FileSpan.newBuilder();
                fileSpanBuilder.setBeg(first.getPos());
                fileSpanBuilder.setEnd(next.getRight().getPos());
                return Either.left(new Ast(fileSpanBuilder.build(), children));
            }
        }
    }
}
