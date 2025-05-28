import newbee

def main():
    print(newbee.sum_as_string(1,2))
    
    l = [0,1,2,3,4,5,6,7,8,9,10]
    print(newbee.sum_as_list(l))
    
    m = {1:2, 3:4, 5:6, 7:8, 9:10}
    print(newbee.sum_as_map(m))

if __name__ == "__main__":
    main()