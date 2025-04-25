for i in src/app/entities/*; do
    i=$(realpath $i)
    if ! grep -q "sea_orm(column_name" $i; then
        continue
    fi
    echo $i
    fields=$(cat $i | grep "sea_orm(column_name"| awk '{print $3}' | cut -d ')' -f1 | sed 's/"//g')
    for j in $fields; do
        ./add_rs $i $j
    done
done
