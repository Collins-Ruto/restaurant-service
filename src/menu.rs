use near_sdk::{env};

pub fn menu() {
    env::log_str("\n
            BEEF
                Striploin steak             $40.0, 
                T-Bone steak                $37.0, 
                Centre Cut Fillet           $35.0, 
                Tomahawk Ribeye steak       $75.0, 
            TOPPINGS
                Fried egg                   $3.0, 
                Prawns                      $5.0, 
                Sauteed mushrooms           $4.0, 
                Onion rings                 $3.50, 
            SIDES       
                Roast baby potatoes         $7.0, 
                Sweet potato fries          $6.50, 
                Roast veggies               $7.0, 
                Tender greens salad         $6.50, 
            DESSERTS
                Baked Chocolate Chip Cookie $8.0, 
                Chocolate Cake              $8.0, 
                Traditional Apple Pie       $8.50, 
                ,Selection of Ice-cream      $2.0, 
        ")
}