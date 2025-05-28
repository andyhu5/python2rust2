import newbee

def main():
    print(newbee.sum_as_string(1,2))
    print(newbee.hello())
    
    c = newbee.Counter(10)
    c.increment()
    c.increment()
    print(c.get_value())
    
    try:
        print(newbee.divide(10, 2))
    except ZeroDivisionError as e:
        print(f"Error: {e}")

    try:
        # calling a rust function that called python function
        print(newbee.call_python_function(6))
    except Exception as e:
        print(f"Error: {e}")

 
if __name__ == "__main__":
    main()