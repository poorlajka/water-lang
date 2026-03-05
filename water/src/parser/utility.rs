use crate::parser::ast::Node;
use crate::parser::token_stream::TokenStream;
use logos::Span;

pub fn create_node<T>(token_stream: &mut TokenStream, span: Span, kind: T) -> Node<T> {
    let id = token_stream.next_id();
    Node::<T>::new(id, span, kind)
}

pub fn span_from_to(start: Span, end: Span) -> Span {
    start.start..end.end
}

pub fn span_from_to_node<T>(start: &Node<T>, end: &Node<T>) -> Span {
    start.span.start..end.span.end
}
