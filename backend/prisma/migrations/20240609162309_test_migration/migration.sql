-- CreateTable
CREATE TABLE "_SubscriptionToUser" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_SubscriptionToUser_AB_unique" ON "_SubscriptionToUser"("A", "B");

-- CreateIndex
CREATE INDEX "_SubscriptionToUser_B_index" ON "_SubscriptionToUser"("B");

-- AddForeignKey
ALTER TABLE "_SubscriptionToUser" ADD CONSTRAINT "_SubscriptionToUser_A_fkey" FOREIGN KEY ("A") REFERENCES "Subscription"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_SubscriptionToUser" ADD CONSTRAINT "_SubscriptionToUser_B_fkey" FOREIGN KEY ("B") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;
