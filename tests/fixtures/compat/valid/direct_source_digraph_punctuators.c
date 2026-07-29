%:define VALUE_7 7
%:define STR(value) %:value
%:define CAT(left, right) left %:%: right

%:if VALUE_7 == 7
%:define CONDITION 1
%:else
%:define CONDITION 0
%:endif

int main(void) <%
    int values<:2:> = <%3, 5%>;
    char *open = STR(<:);
    char *hash = STR(%:);

    return values<:1:> == 5
            && CAT(VALUE_, 7) == 7
            && CONDITION == 1
            && open[0] == '<' && open[1] == ':' && open[2] == '\0'
            && hash[0] == '%' && hash[1] == ':' && hash[2] == '\0'
        ? 0
        : 1;
%>