import ballerina/http;

# Calculation request.
#
# + operand1 - Is a float used as the first operand in an equation
# + operand2 - Is a float used as the second operand in an equation
# + operator - Is a string that represents the operator
public type Calculation record {|
    float operand1;
    float operand2;
    string operator;
|};

# Response to calculation request.
#
# + result - The result of the operation
# + expression - The evaluated expression that used to calculate the result
public type Response record {|
    float result;
    string expression;
|};

service / on new http:Listener(9090) {
    
    # Calculates result based on the given calculation request.
    #
    # + request - Calculation request
    # + return - Calculation response
    resource function post calc(Calculation request) returns Response {
        return {
            result: calculate(request),
            expression: buildExpression(request)
        };
    }
}

function calculate(Calculation calc) returns float {
    match calc.operator {
        "+" => {
            return calc.operand1 + calc.operand2;
        }
        "-" => {
            return calc.operand1 - calc.operand2;
        }
        "x"|"*" => {
            return calc.operand1 * calc.operand2;
        }
        "/" => {
            return calc.operand1 / calc.operand2;
        }
        _ => {
            return 0.0;
        }
    }
}

function buildExpression(Calculation calc) returns string {
    return string `${calc.operand1}${calc.operator}${calc.operand2}`;
}
